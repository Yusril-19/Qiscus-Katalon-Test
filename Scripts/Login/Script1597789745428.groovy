import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('https://multichannel.qiscus.com/')

WebUI.setText(findTestObject('Object Repository/Page_Qiscus Multichannel/input_Work Email_email'), 'qiscus1@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Qiscus Multichannel/input_Forgot Password_password'), 'Gelr1u/q0kY=')

WebUI.click(findTestObject('Object Repository/Page_Qiscus Multichannel/button_Sign In'))

WebUI.click(findTestObject('Object Repository/Page_Verified Email/div_Qiscus 1_icon'))

WebUI.navigateToUrl('https://multichannel.qiscus.com/')

WebUI.setText(findTestObject('Object Repository/Page_Qiscus Multichannel/input_Work Email_email'), 'qiscus2@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Qiscus Multichannel/input_Forgot Password_password'), 'FZk3nD3s/f8=')

WebUI.click(findTestObject('Object Repository/Page_Qiscus Multichannel/button_Sign In'))

WebUI.click(findTestObject('Object Repository/Page_Verified Email/div_Qiscus 1_icon'))

WebUI.navigateToUrl('https://multichannel.qiscus.com/')

WebUI.setText(findTestObject('Object Repository/Page_Qiscus Multichannel/input_Work Email_email'), 'qiscustest@emailtech.info')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Qiscus Multichannel/input_Forgot Password_password'), '0MvknSbVDbffUzAMtJmjdA==')

WebUI.click(findTestObject('Object Repository/Page_Qiscus Multichannel/button_Sign In'))

WebUI.getUrl()

WebUI.closeBrowser()

